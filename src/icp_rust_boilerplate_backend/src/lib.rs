#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use std::borrow::{Borrow, BorrowMut};


type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Car {
    id: u64,
    make: String,
    model: String,
    year: u32,
    color: String,
    created_at: u64,
    updated_at: Option<u64>,
    owner: String,
    is_booked: bool, // New field for booking status
}

impl Storable for Car {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Car {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static CAR_STORAGE: RefCell<StableBTreeMap<u64, Car, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct CarPayload {
    make: String,
    model: String,
    year: u32,
    color: String,
    owner: String,
    is_booked: bool, // Add is_booked field to payload
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Customer {
    id: u64,
    name: String,
    contact: String,
}

impl Storable for Customer {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Customer {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Reservation {
    car_id: u64,
    customer_id: u64,
    reservation_time: u64,
}

impl Storable for Reservation {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Reservation {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[ic_cdk::query]
fn get_car(id: u64) -> Result<Car, Error> {
    match _get_car(&id) {
        Some(car) => Ok(car),
        None => Err(Error::NotFound {
            msg: format!("a car with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn add_car(car: CarPayload) -> Option<Car> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let car = Car {
        id,
        make: car.make,
        model: car.model,
        year: car.year,
        color: car.color,
        created_at: time(),
        updated_at: None,
        owner: car.owner,
        is_booked: car.is_booked, // Set is_booked from payload
    };
    do_insert_car(&car);
    Some(car)
}

#[ic_cdk::update]
fn update_car(id: u64, payload: CarPayload) -> Result<Car, Error> {
    match CAR_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut car) => {
            car.make = payload.make;
            car.model = payload.model;
            car.year = payload.year;
            car.color = payload.color;
            car.updated_at = Some(time());
            car.owner = payload.owner;
            car.is_booked = payload.is_booked; // Update is_booked field
            do_insert_car(&car);
            Ok(car)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a car with id={}. car not found",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn is_booked(id: u64) -> Result<bool, Error> {
    match _get_car(&id) {
        Some(car) => Ok(car.is_booked),
        None => Err(Error::NotFound {
            msg: format!("a car with id={} not found", id),
        }),
    }
}

fn do_insert_car(car: &Car) {
    CAR_STORAGE.with(|service| service.borrow_mut().insert(car.id, car.clone()));
}

#[ic_cdk::update]
fn delete_car(id: u64) -> Result<Car, Error> {
    match CAR_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(car) => Ok(car),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a car with id={}. car not found.",
                id
            ),
        }),
    }
}

#[ic_cdk::update]
fn add_customer(name: String, contact: String) -> Option<Customer> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let customer = Customer {
        id,
        name,
        contact,
    };
    do_insert_customer(&customer);
    Some(customer)
}

fn do_insert_customer(customer: &Customer) {
    // Assuming MemoryId::new(2) is reserved for customer storage
    let customer_storage = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)));
    StableBTreeMap::<u64, Customer, Memory>::init(customer_storage)
        .borrow_mut()
        .insert(customer.id, customer.clone());
}

#[ic_cdk::query]
fn get_customer(id: u64) -> Result<Customer, Error> {
    match _get_customer(&id) {
        Some(customer) => Ok(customer),
        None => Err(Error::NotFound {
            msg: format!("a customer with id={} not found", id),
        }),
    }
}

fn _get_customer(id: &u64) -> Option<Customer> {
    // Assuming MemoryId::new(2) is reserved for customer storage
    let customer_storage = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)));
    StableBTreeMap::<u64, Customer, Memory>::init(customer_storage)
        .borrow()
        .get(id)
}

#[ic_cdk::update]
fn delete_customer(id: u64) -> Result<Customer, Error> {
    match _get_customer(&id) {
        Some(customer) => {
            // Assuming MemoryId::new(2) is reserved for customer storage
            let customer_storage = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)));
            StableBTreeMap::<u64, Customer, Memory>::init(customer_storage)
                .borrow_mut()
                .remove(&id);
            Ok(customer)
        }
        None => Err(Error::NotFound {
            msg: format!("a customer with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn make_reservation(car_id: u64, customer_id: u64) -> Result<Reservation, Error> {
    match (_get_car(&car_id), _get_customer(&customer_id)) {
        (Some(_), Some(_)) => {
            let reservation = Reservation {
                car_id,
                customer_id,
                reservation_time: time(),
            };
            do_insert_reservation(&reservation);
            Ok(reservation)
        }
        _ => Err(Error::NotFound {
            msg: "Car or customer not found for reservation".to_string(),
        }),
    }
}

fn do_insert_reservation(reservation: &Reservation) {
    // Assuming MemoryId::new(3) is reserved for reservation storage
    let reservation_storage = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)));
    
    StableBTreeMap::<u64, Reservation, Memory>::init(reservation_storage)
        .borrow_mut()
        .insert(reservation.car_id, reservation.clone());
}


#[ic_cdk::query]
fn get_reservation(car_id: u64) -> Result<Reservation, Error> {
    match _get_reservation(&car_id) {
        Some(reservation) => Ok(reservation),
        None => Err(Error::NotFound {
            msg: format!("a reservation for car_id={} not found", car_id),
        }),
    }
}

fn _get_reservation(car_id: &u64) -> Option<Reservation> {
    // Assuming MemoryId::new(3) is reserved for reservation storage
    let reservation_storage = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)));
    StableBTreeMap::<u64, Reservation, Memory>::init(reservation_storage)
        .borrow()
        .get(car_id)
}

#[ic_cdk::update]
fn cancel_reservation(car_id: u64) -> Result<(), Error> {
    match _get_reservation(&car_id) {
        Some(_) => {
            // Assuming MemoryId::new(3) is reserved for reservation storage
            let reservation_storage = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)));
            StableBTreeMap::<u64, Reservation, Memory>::init(reservation_storage)
                .borrow_mut()
                .remove(&car_id);
            Ok(())
        }
        None => Err(Error::NotFound {
            msg: format!("a reservation for car_id={} not found", car_id),
        }),
    }
}

#[ic_cdk::query]
fn generate_report() -> Vec<Car> {
    // Assuming MemoryId::new(1) is reserved for car storage
    let car_storage = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)));
    StableBTreeMap::<u64, Car, Memory>::init(car_storage)
        .borrow()
        .iter()
        .map(|(_, car)| car.clone())
        .collect()
}


#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

fn _get_car(id: &u64) -> Option<Car> {
    // Assuming MemoryId::new(1) is reserved for car storage
    let car_storage = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)));
    StableBTreeMap::<u64, Car, Memory>::init(car_storage)
        .borrow()
        .get(id)
}

ic_cdk::export_candid!();
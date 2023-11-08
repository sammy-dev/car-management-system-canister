#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

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
    do_insert(&car);
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
            do_insert(&car);
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

fn do_insert(car: &Car) {
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

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

fn _get_car(id: &u64) -> Option<Car> {
    CAR_STORAGE.with(|service| service.borrow().get(id))
}

ic_cdk::export_candid!();

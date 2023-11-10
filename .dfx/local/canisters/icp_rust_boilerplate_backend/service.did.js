export const idlFactory = ({ IDL }) => {
  const CarPayload = IDL.Record({
    'is_booked' : IDL.Bool,
    'model' : IDL.Text,
    'owner' : IDL.Text,
    'make' : IDL.Text,
    'color' : IDL.Text,
    'year' : IDL.Nat32,
  });
  const Car = IDL.Record({
    'id' : IDL.Nat64,
    'is_booked' : IDL.Bool,
    'model' : IDL.Text,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'owner' : IDL.Text,
    'make' : IDL.Text,
    'color' : IDL.Text,
    'year' : IDL.Nat32,
    'created_at' : IDL.Nat64,
  });
  const Customer = IDL.Record({
    'id' : IDL.Nat64,
    'contact' : IDL.Text,
    'name' : IDL.Text,
  });
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  const Reservation = IDL.Record({
    'reservation_time' : IDL.Nat64,
    'customer_id' : IDL.Nat64,
    'car_id' : IDL.Nat64,
  });
  return IDL.Service({
    'add_car' : IDL.Func([CarPayload], [IDL.Opt(Car)], []),
    'add_customer' : IDL.Func([IDL.Text, IDL.Text], [IDL.Opt(Customer)], []),
    'cancel_reservation' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error })],
        [],
      ),
    'delete_car' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : Car, 'Err' : Error })],
        [],
      ),
    'delete_customer' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : Customer, 'Err' : Error })],
        [],
      ),
    'generate_report' : IDL.Func([], [IDL.Vec(Car)], []),
    'get_car' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : Car, 'Err' : Error })],
        ['query'],
      ),
    'get_customer' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : Customer, 'Err' : Error })],
        ['query'],
      ),
    'get_reservation' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : Reservation, 'Err' : Error })],
        ['query'],
      ),
    'is_booked' : IDL.Func(
        [IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : Error })],
        ['query'],
      ),
    'make_reservation' : IDL.Func(
        [IDL.Nat64, IDL.Nat64],
        [IDL.Variant({ 'Ok' : Reservation, 'Err' : Error })],
        [],
      ),
    'update_car' : IDL.Func(
        [IDL.Nat64, CarPayload],
        [IDL.Variant({ 'Ok' : Car, 'Err' : Error })],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };

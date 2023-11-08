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
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  const Result = IDL.Variant({ 'Ok' : Car, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : Error });
  return IDL.Service({
    'add_car' : IDL.Func([CarPayload], [IDL.Opt(Car)], []),
    'delete_car' : IDL.Func([IDL.Nat64], [Result], []),
    'get_car' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'is_booked' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'update_car' : IDL.Func([IDL.Nat64, CarPayload], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };

import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Car {
  'id' : bigint,
  'is_booked' : boolean,
  'model' : string,
  'updated_at' : [] | [bigint],
  'owner' : string,
  'make' : string,
  'color' : string,
  'year' : number,
  'created_at' : bigint,
}
export interface CarPayload {
  'is_booked' : boolean,
  'model' : string,
  'owner' : string,
  'make' : string,
  'color' : string,
  'year' : number,
}
export interface Customer { 'id' : bigint, 'contact' : string, 'name' : string }
export type Error = { 'NotFound' : { 'msg' : string } };
export interface Reservation {
  'reservation_time' : bigint,
  'customer_id' : bigint,
  'car_id' : bigint,
}
export interface _SERVICE {
  'add_car' : ActorMethod<[CarPayload], [] | [Car]>,
  'add_customer' : ActorMethod<[string, string], [] | [Customer]>,
  'delete_car' : ActorMethod<[bigint], { 'Ok' : Car } | { 'Err' : Error }>,
  'generate_report' : ActorMethod<[], Array<Car>>,
  'get_car' : ActorMethod<[bigint], { 'Ok' : Car } | { 'Err' : Error }>,
  'get_customer' : ActorMethod<
    [bigint],
    { 'Ok' : Customer } |
      { 'Err' : Error }
  >,
  'get_reservation' : ActorMethod<
    [bigint],
    { 'Ok' : Reservation } |
      { 'Err' : Error }
  >,
  'is_booked' : ActorMethod<[bigint], { 'Ok' : boolean } | { 'Err' : Error }>,
  'make_reservation' : ActorMethod<
    [bigint, bigint],
    { 'Ok' : Reservation } |
      { 'Err' : Error }
  >,
  'update_car' : ActorMethod<
    [bigint, CarPayload],
    { 'Ok' : Car } |
      { 'Err' : Error }
  >,
}

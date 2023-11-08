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
export type Error = { 'NotFound' : { 'msg' : string } };
export type Result = { 'Ok' : Car } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : boolean } |
  { 'Err' : Error };
export interface _SERVICE {
  'add_car' : ActorMethod<[CarPayload], [] | [Car]>,
  'delete_car' : ActorMethod<[bigint], Result>,
  'get_car' : ActorMethod<[bigint], Result>,
  'is_booked' : ActorMethod<[bigint], Result_1>,
  'update_car' : ActorMethod<[bigint, CarPayload], Result>,
}

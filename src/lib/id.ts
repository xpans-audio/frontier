export class TaskIDManager {
  _last_id: number = 0;
  get new_id(): number {
    this._last_id += 1;
    return this._last_id;
  }
}

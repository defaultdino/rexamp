pub enum LoadState<T> {
    Idle,
    Loading,
    Loaded(T),
    Failed,
}

# chat相关问题
## 通过AppState来实现model相关查询方法，是否合理？
如果model相关类很多的话，这样AppState对应的方法也会很多。这个出现方法名冲突，而且使用起来也很不方便。appstate职责太多。
是不是应该将pgpool作为一个全局变量来使用，针对每个domain来实现数据库相关的查询。

## AppState 定义问题
```rust
#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
pub struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) dk: DecodingKey,
    pub(crate) ek: EncodingKey,
    pub(crate) pool: PgPool,
}
```
为啥不直接包裹
```rust
pub struct AppState {
    pub(crate) config: AppConfig,
    pub(crate) dk: DecodingKey,
    pub(crate) ek: EncodingKey,
    pub(crate) pool: PgPool,
}
```

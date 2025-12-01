# Начало работы

Это руководство поможет вам начать работу с `protofolio` - библиотекой Rust для генерации спецификаций AsyncAPI 3.0 из вашего кода.

## Установка и требования

### Требования

- **Rust**: 1.80 или новее
- **Зависимости**: Ваши типы сообщений должны реализовывать:
  - `Serialize` и `Deserialize` (из `serde`)
  - `JsonSchema` (из `schemars`)

### Установка

Добавьте в ваш `Cargo.toml`:

```toml
[dependencies]
protofolio = "0.1.0"
protofolio-derive = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
schemars = { version = "1.0", features = ["derive"] }
```

## Быстрый старт

Вот минимальный пример для начала:

```rust
use protofolio::AsyncApi;
use protofolio_derive::{AsyncApi, AsyncApiMessage};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Определите ваш тип сообщения
#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "events", messageId = "event-v1")]
pub struct Event {
    pub id: String,
    pub data: String,
}

// Определите вашу спецификацию AsyncAPI
#[derive(AsyncApi)]
#[asyncapi(
    info(title = "My API", version = "1.0.0"),
    channels("events"),
    messages(Event)
)]
pub struct MyApi;

// Сгенерируйте спецификацию
let spec = MyApi::asyncapi();
let json = MyApi::asyncapi_json()?;
```

## Что дальше?

- Проверьте [Примеры](/ru/examples/basic) - более подробные примеры
- Прочитайте [Руководство по сообщениям](/ru/guides/messages) - как определять и настраивать типы сообщений
- Изучите [Руководство по операциям](/ru/guides/operations) - как определять операции публикации/подписки


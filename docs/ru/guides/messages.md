# Сообщения

> ⚠️ **Перевод в процессе**: Эта страница еще не полностью переведена. Пожалуйста, обратитесь к [английской версии](/guides/messages) для полной документации.

## Введение

Сообщения в AsyncAPI представляют данные, передаваемые через каналы. В `protofolio` вы определяете сообщения как обычные структуры Rust.

## Базовое использование

```rust
use protofolio_derive::AsyncApiMessage;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, AsyncApiMessage)]
#[asyncapi(channel = "events", messageId = "event-v1")]
pub struct Event {
    pub id: String,
    pub data: String,
}
```

## Атрибуты сообщений

- `channel` - канал, на котором передается сообщение
- `messageId` - уникальный идентификатор сообщения (рекомендуется включать версию)

[Полная документация на английском](/guides/messages)


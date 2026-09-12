#![warn(missing_docs)]
//! Message types for inter-component communication.
//!
//! Inherits the canonical message model from `themql-message` (theMQL),
//! then adds theLiGI-specific envelope, event, command, and notification
//! types used throughout the platform.

pub use themql_message::{
    CausationId, CorrelationId, Message as ThemqlMessage, MessageId, Metadata, Operation, Payload,
    Subject,
};

use serde::{Deserialize, Serialize};
use theligi_core::SchemaVersion;

/// A message envelope wrapping payload with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEnvelope<T> {
    /// Correlation id.
    pub correlation_id: CorrelationId,
    /// Causation id.
    pub causation_id: CausationId,
    /// Schema version of the payload.
    pub schema_version: SchemaVersion,
    /// The actual payload.
    pub payload: T,
}

impl<T> MessageEnvelope<T> {
    /// Create a new `MessageEnvelope`.
    pub fn new(
        correlation_id: CorrelationId,
        causation_id: CausationId,
        schema_version: SchemaVersion,
        payload: T,
    ) -> Self {
        Self {
            correlation_id,
            causation_id,
            schema_version,
            payload,
        }
    }
}

/// An event representing something that happened.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Identity of the event.
    pub identity: String,
    /// Event payload.
    pub payload: serde_json::Value,
}

impl Event {
    /// Create a new `Event`.
    pub fn new(identity: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            identity: identity.into(),
            payload,
        }
    }
}

/// A command requesting an action be performed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Identity of the command.
    pub identity: String,
    /// Target resource.
    pub target: String,
    /// Command payload.
    pub payload: serde_json::Value,
}

impl Command {
    /// Create a new `Command`.
    pub fn new(
        identity: impl Into<String>,
        target: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            identity: identity.into(),
            target: target.into(),
            payload,
        }
    }
}

/// A notification sent to interested parties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Identity of the notification.
    pub identity: String,
    /// Notification payload.
    pub payload: serde_json::Value,
}

impl Notification {
    /// Create a new `Notification`.
    pub fn new(identity: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            identity: identity.into(),
            payload,
        }
    }
}

/// A message in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    /// An event message.
    Event(Event),
    /// A command message.
    Command(Command),
    /// A notification message.
    Notification(Notification),
}

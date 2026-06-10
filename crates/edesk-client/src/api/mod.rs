//! Typed operations for every eDesk API resource, implemented as methods on
//! [`crate::Client`].

mod channels;
mod contacts;
mod messages;
mod order_notes;
mod sales_orders;
mod system;
mod tag_groups;
mod tags;
mod templates;
mod tickets;
mod tracking_links;
mod users;

pub use contacts::ListContactsParams;
pub use messages::{CreateMessageRequest, MessageAttachment, UpdateMessageRequest};
pub use order_notes::{CreateOrderNoteRequest, NoteFile, UpdateOrderNoteRequest};
pub use sales_orders::ListSalesOrdersParams;
pub use tags::TagRequest;
pub use tickets::{
    ContactRequest, CreateTicketRequest, CustomField, ListTicketsParams, UpdateTicketRequest,
};
pub use tracking_links::TrackingLink;

use crate::resources::tickets::models::{Ticket, TicketWrapper};
use crate::Client;

/// # Errors
///
/// Will return `reqwest::Error` if the HTTP request fails.
pub async fn get_ticket(client: Client, ticket_number: &str) -> Result<Ticket, reqwest::Error> {
    let ticket_wrapper = client
        .make_request::<TicketWrapper>(
            reqwest::Method::GET,
            format!("tickets/{ticket_number}.json"),
        )
        .await?;
    Ok(ticket_wrapper.ticket)
}

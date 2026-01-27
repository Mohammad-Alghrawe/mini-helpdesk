use crate::models::ticket::{
    CreateTicketRequest, Ticket, TicketPriority, TicketStatus, UpdateTicketRequest,
};
use sqlx::Row;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use uuid::Uuid;

fn now_iso() -> String {
    // RFC3339, e.g. "2026-01-27T10:00:00Z"
    OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| OffsetDateTime::now_utc().unix_timestamp().to_string())
}

fn priority_to_str(p: &TicketPriority) -> &'static str {
    match p {
        TicketPriority::Low => "low",
        TicketPriority::Medium => "medium",
        TicketPriority::High => "high",
    }
}

fn status_to_str(s: &TicketStatus) -> &'static str {
    match s {
        TicketStatus::Open => "open",
        TicketStatus::InProgress => "in_progress",
        TicketStatus::Closed => "closed",
    }
}

fn str_to_priority(s: &str) -> TicketPriority {
    match s {
        "low" => TicketPriority::Low,
        "high" => TicketPriority::High,
        _ => TicketPriority::Medium,
    }
}

fn str_to_status(s: &str) -> TicketStatus {
    match s {
        "in_progress" => TicketStatus::InProgress,
        "closed" => TicketStatus::Closed,
        _ => TicketStatus::Open,
    }
}

pub async fn update_ticket(
    pool: &sqlx::SqlitePool,
    id: &str,
    payload: UpdateTicketRequest,
) -> Result<Option<Ticket>, sqlx::Error> {
    // Convert enums -> strings (match your DB values)
    let status_str = payload.status.as_ref().map(status_to_str);
    let priority_str = payload.priority.as_ref().map(priority_to_str);

    let row = sqlx::query(
        r#"
        UPDATE tickets
        SET
          status     = COALESCE(?, status),
          priority   = COALESCE(?, priority),
          updated_at = STRFTIME('%Y-%m-%dT%H:%M:%fZ','now')
        WHERE id = ?
        RETURNING id, title, description, priority, status, created_at, updated_at
        "#,
    )
    .bind(status_str)
    .bind(priority_str)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Ticket {
        id: r.get::<String, _>("id"),
        title: r.get::<String, _>("title"),
        description: r.get::<Option<String>, _>("description"),
        priority: str_to_priority(&r.get::<String, _>("priority")),
        status: str_to_status(&r.get::<String, _>("status")),
        created_at: r.get::<String, _>("created_at"),
        updated_at: r.get::<String, _>("updated_at"),
    }))
}

pub async fn create_ticket(
    pool: &SqlitePool,
    req: CreateTicketRequest,
) -> Result<Ticket, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let created_at = now_iso();
    let updated_at = created_at.clone();

    let priority = req.priority.unwrap_or_default();
    let status = TicketStatus::default();

    sqlx::query(
        r#"
        INSERT INTO tickets (id, title, description, priority, status, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(priority_to_str(&priority))
    .bind(status_to_str(&status))
    .bind(&created_at)
    .bind(&updated_at)
    .execute(pool)
    .await?;

    Ok(Ticket {
        id,
        title: req.title,
        description: req.description,
        priority,
        status,
        created_at,
        updated_at,
    })
}

pub async fn list_tickets(pool: &SqlitePool) -> Result<Vec<Ticket>, sqlx::Error> {
    // Runtime query (no DATABASE_URL needed at compile time)
    let rows = sqlx::query(
        r#"
        SELECT id, title, description, priority, status, created_at, updated_at
        FROM tickets
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    let tickets = rows
        .into_iter()
        .map(|r| Ticket {
            id: r.get::<String, _>("id"),
            title: r.get::<String, _>("title"),
            description: r.get::<Option<String>, _>("description"),
            priority: str_to_priority(&r.get::<String, _>("priority")),
            status: str_to_status(&r.get::<String, _>("status")),
            created_at: r.get::<String, _>("created_at"),
            updated_at: r.get::<String, _>("updated_at"),
        })
        .collect();

    Ok(tickets)
}

pub async fn get_ticket_by_id(
    pool: &sqlx::SqlitePool,
    id: &str,
) -> Result<Option<Ticket>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, title, description, priority, status, created_at, updated_at
        FROM tickets
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Ticket {
        id: r.get::<String, _>("id"),
        title: r.get::<String, _>("title"),
        description: r.get::<Option<String>, _>("description"),
        priority: str_to_priority(&r.get::<String, _>("priority")),
        status: str_to_status(&r.get::<String, _>("status")),
        created_at: r.get::<String, _>("created_at"),
        updated_at: r.get::<String, _>("updated_at"),
    }))
}

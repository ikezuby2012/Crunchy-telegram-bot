use crate::models::soccer::TodayApiResponse;

fn format_events(events: TodayApiResponse) -> String {
    if events.events.is_empty() {
        return "No events scheduled for today.".to_string();
    }

    let mut message = String::from("Today's events:\n\n");
    for event in events.events {
        message.push_str(&format!(
            "- {:?} vs {:?} \n",
            event.home_team, event.away_team
        ));
    }
    message
}

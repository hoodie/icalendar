use icalendar::*;
use jiff::{Timestamp, ToSpan as _};

fn main() {
    // alarm will occur one minute from now
    let event_with_absolute_audio_alarm = Event::new()
        .alarm(Alarm::audio(Timestamp::now() + 1.minute()).duration_and_repeat(1.minute(), 4))
        .done();

    // alarm will occur one minute before the start
    let event_with_relative_display_alarm = Event::new()
        .alarm(Alarm::display("ALARM! ALARM!", -1.minute()).duration_and_repeat(1.minute(), 4))
        .done();
    // alarm will occur one minute before the end
    let event_with_relative_display_alarm_end = Event::new()
        .alarm(
            Alarm::display("ALARM! ALARM!", (-1.minute(), Related::End))
                .duration_and_repeat(1.minute(), 4),
        )
        .done();
    event_with_absolute_audio_alarm.print().unwrap();
    event_with_relative_display_alarm.print().unwrap();
    event_with_relative_display_alarm_end.print().unwrap();
}

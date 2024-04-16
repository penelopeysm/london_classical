export function formatDate(date: Date): string {
    let day_of_week = date.toLocaleString(undefined, { weekday: 'long' });
    let date_long = date.toLocaleString(undefined, { day: '2-digit', month: 'long', year: 'numeric' });
    let time = date.toLocaleString(undefined, { hour: 'numeric', minute: '2-digit', hour12: true });
    return `${date_long} (${day_of_week}), ${time}`;
}

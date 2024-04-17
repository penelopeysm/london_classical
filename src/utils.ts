import { type Concert } from 'src/lib/bindings/Concert';

export function formatDate(date: Date): string {
    let day_of_week = date.toLocaleString(undefined, { weekday: 'long' });
    let date_long = date.toLocaleString(undefined, { day: 'numeric', month: 'long', year: 'numeric' });
    let time = date.toLocaleString(undefined, { hour: 'numeric', minute: '2-digit', hour12: true });
    return `${date_long} (${day_of_week}), ${time}`;
}

export function getPriceString(concert: Concert): string {
    if (concert.min_price !== null && concert.max_price !== null) {
        if (concert.min_price === concert.max_price) {
            if (concert.min_price === 0) {
                return "Free entry";
            }
            else {
                return `£${concert.min_price / 100}`;
            }
        } else {
            return `£${concert.min_price / 100}–£${concert.max_price / 100}`;
        }
    }
    return "Price not available";
}

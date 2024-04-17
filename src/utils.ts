import { type Concert } from 'src/lib/bindings/Concert';

export type HashedConcert = Concert & { hash: string };

export function hashConcert(concert: Concert): HashedConcert {
    let hash = 0;
    let hashString = `${concert.title}_${concert.venue}_${concert.datetime}`;
    for (let i = 0; i < hashString.length; i++) {
        let char = hashString.charCodeAt(i);
        hash = ((hash << 5) - hash) + char;
        hash |= 0;
    }
    return {
        ...concert,
        hash: (hash >>> 0).toString(16)
    }
}

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

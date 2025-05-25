import { type Concert } from 'src/lib/bindings/Concert';

export function formatDate(date: Date): string {
    let day_of_week = date.toLocaleString(undefined, { weekday: 'long' });
    let date_long = date.toLocaleString(undefined, { day: 'numeric', month: 'long', year: 'numeric' });
    let time = date.toLocaleString(undefined, { hour: 'numeric', minute: '2-digit', hour12: true });
    return `${date_long} (${day_of_week}), ${time}`;
}

function toPounds(price: number): string {
    if (price % 100 === 0) {
        return `£${price / 100}`;
    } else {
        return `£${Math.floor(price / 100)}.${(price % 100).toString().padStart(2, '0')}`;
    }
}

export function getPriceString(concert: Concert): string {
    if (concert.min_price !== null && concert.max_price !== null) {
        if (concert.min_price === concert.max_price) {
            if (concert.min_price === 0) {
                return "Free entry";
            }
            else {
                return `${toPounds(concert.min_price)} `;
            }
        } else {
            return `${toPounds(concert.min_price)}–${toPounds(concert.max_price)} `;
        }
    }
    else if (concert.min_price !== null && concert.max_price === null) {
        return `from ${toPounds(concert.min_price)} `;
    }
    else {
        return "Price not available";
    }
}

export function notUndefined<T>(x: T | undefined): T {
    if (x === undefined) {
        throw new Error("Unexpected undefined value");
    }
    return x;
}

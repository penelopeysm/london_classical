import { get, writable, type Writable } from 'svelte/store';
import { type Concert } from "src/lib/bindings/Concert";
import { initialFilters, type FiltersType } from "src/lib/filters";

// Concerts and their views
type ConcertViews = Map<string, Concert[]>;
import allConcerts from "src/assets/concerts.json";
const defaultViewName = "All";
const localStorageKey = "local_views";

function loadViewsFromLocalStorage(): ConcertViews {
    const views = localStorage.getItem(localStorageKey);
    let allViews = new Map<string, Concert[]>();
    allViews.set(defaultViewName, allConcerts);
    if (views !== null) {
        for (const [key, value] of Object.entries(JSON.parse(views))) {
            if (key !== defaultViewName) {
                let thisViewConcertIds = JSON.parse(value as string) as string[];
                // Fetch the full concerts from allConcerts
                const thisViewConcerts = thisViewConcertIds.map((concertId) => {
                    return allConcerts.find((concert) => concert.id === concertId);
                })
                    .filter((concert) => concert !== undefined && new Date(concert.datetime) > new Date());
                allViews.set(key, thisViewConcerts as Concert[]);
            }
        }
    }
    return allViews;
}

function storeViewsInLocalStorage(views: ConcertViews) {
    const viewsToStore: { [key: string]: string } = {};
    for (const [viewName, concerts] of views) {
        if (viewName !== defaultViewName) {
            const concertIds = concerts.map((concert) => concert.id);
            viewsToStore[viewName] = JSON.stringify(concertIds);
        }
    }
    localStorage.setItem(localStorageKey, JSON.stringify(viewsToStore));
}

// Generate a store that syncs with localStorage
const { subscribe, set, update }: Writable<ConcertViews> = writable(loadViewsFromLocalStorage());
export const concertViews = {
    subscribe: subscribe,
    set: (views: ConcertViews) => {
        storeViewsInLocalStorage(views);
        set(views);
    },
    update: (cb: (views: ConcertViews) => ConcertViews) => {
        const newViews = cb(get(concertViews));
        storeViewsInLocalStorage(newViews);
        update(() => newViews);
    }
}

export const currentViewName: Writable<string> = writable(defaultViewName);

// Indices of selected concerts
export const selectedConcertIndices: Writable<number[]> = writable([]);

// Store for state of filters. We have to make a copy of initialFilters to avoid
// mutating it
const initialFiltersCopy = JSON.parse(JSON.stringify(initialFilters));
export const filters: Writable<FiltersType> = writable(initialFiltersCopy);

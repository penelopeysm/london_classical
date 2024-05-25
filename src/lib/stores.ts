import { get, writable, type Writable } from 'svelte/store';
import { type Concert } from "src/lib/bindings/Concert";
import { initialFilters, type FiltersType } from "src/lib/filters";

// Concerts and their views
type ConcertViews = Map<string, Concert[]>;
import allConcerts from "src/assets/concerts.json";
export const defaultViewName = "All";
const localStorageKey = "local_views";

// This function expects { "viewName": ["concertId1", "concertId2", ...], ... }
export function viewFromJson(viewJson: string): ConcertViews {
    let views = new Map<string, Concert[]>();
    for (const [key, values] of Object.entries(JSON.parse(viewJson))) {
        if (key !== defaultViewName) {
            let thisViewConcertIds = [...values as string];
            // Fetch the full concerts from allConcerts
            const thisViewConcerts = thisViewConcertIds.map((concertId) => {
                return allConcerts.find((concert) => concert.id === concertId);
            })
                .filter((concert) => concert !== undefined && new Date(concert.datetime) > new Date());
            views.set(key, thisViewConcerts as Concert[]);
        }
    }
    return views;
}

function loadViewsFromLocalStorage(): ConcertViews {
    const views = localStorage.getItem(localStorageKey);
    let allViews = new Map<string, Concert[]>();
    allViews.set(defaultViewName, allConcerts);
    if (views !== null) {
        for (const [key, value] of viewFromJson(views).entries()) {
            allViews.set(key, value);
        }
    }
    return allViews;
}

function storeViewsInLocalStorage(views: ConcertViews) {
    const viewsToStore: { [key: string]: string[] } = {};
    for (const [viewName, concerts] of views) {
        if (viewName !== defaultViewName) {
            const concertIds = concerts.map((concert) => concert.id);
            viewsToStore[viewName] = concertIds;
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
export const selectedConcertIds: Writable<string[]> = writable([]);

// Store for state of filters. We have to make a copy of initialFilters to avoid
// mutating it
const initialFiltersCopy = JSON.parse(JSON.stringify(initialFilters));
export const filters: Writable<FiltersType> = writable(initialFiltersCopy);

import { writable, type Writable } from 'svelte/store';
import { type Concert } from "src/lib/bindings/Concert";
import { initialFilters, type FiltersType } from "src/lib/filters";

// Store for concerts and their views
import allConcerts from "src/assets/concerts.json";
type ConcertViews = Map<string, Concert[]>;
let initialConcertViews = new Map<string, Concert[]>();
const defaultViewName = "All";
initialConcertViews.set(defaultViewName, allConcerts);
export const concerts: Writable<ConcertViews> = writable(initialConcertViews);
export const currentViewName: Writable<string> = writable(defaultViewName);

// Indices of selected concerts
export const selectedConcertIndices: Writable<number[]> = writable([]);

// Store for state of filters. We have to make a copy of initialFilters to avoid
// mutating it
const initialFiltersCopy = JSON.parse(JSON.stringify(initialFilters));
export const filters: Writable<FiltersType> = writable(initialFiltersCopy);

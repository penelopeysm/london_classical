import { type Concert } from "src/lib/bindings/Concert";

export type FiltersType = {
    searchTerm: string;
    booleanTagNames: string[];
};

// Type of a filter which is either on/off
export type BooleanFilter = {
    tagName: string;
    tagColor: string;
    filterFunc: (concert: Concert) => boolean;
}

// List of all boolean filters that we know of
export const allBooleanFilters: BooleanFilter[] = [
    {
        tagName: "Wigmore Hall",
        tagColor: "#17a8ad",
        filterFunc: (concert: Concert) => concert.venue === "Wigmore Hall",
    },
    {
        tagName: "Wigmore U35 £5",
        tagColor: "#3694cf",
        filterFunc: (concert: Concert) => concert.is_wigmore_u35,
    },
    {
        tagName: "BBC Proms",
        tagColor: "#c462f5",
        filterFunc: (concert: Concert) => concert.is_prom,
    },
    {
        tagName: "Royal Albert Hall",
        tagColor: "#c72ac2",
        filterFunc: (concert: Concert) => concert.venue === "Royal Albert Hall",
    }
];

// Check if a concert satisfies the filters
function satisfies(concert: Concert, filters: FiltersType): boolean {
    // Check search filter
    let ciSearchTerm = filters.searchTerm.toLowerCase();
    let searchPass =
        filters.searchTerm === "" ||
        concert.title.toLowerCase().includes(ciSearchTerm) ||
        (concert.subtitle !== null &&
            concert.subtitle.toLowerCase().includes(ciSearchTerm)) ||
        concert.venue.toLowerCase().includes(ciSearchTerm) ||
        concert.performers.some((p) =>
            p.name.toLowerCase().includes(ciSearchTerm),
        );

    // Check boolean tags
    let booleanPass = filters.booleanTagNames.every((tag) => {
        let filter = allBooleanFilters.find((f) => f.tagName === tag);
        if (filter === undefined) {
            console.error(`Unknown boolean tag ${tag}`);
            return false;
        }
        return filter.filterFunc(concert);
    });

    // Return conjunction of both
    return searchPass && booleanPass;
}

export function getPassingIndices(concerts: Concert[], filters: FiltersType): number[] {
    let passingIndices: number[] = [];
    concerts.forEach((concert, i) => {
        if (satisfies(concert, filters)) {
            passingIndices.push(i);
        }
    });
    return passingIndices;
}

export const initialFilters: FiltersType = {
    searchTerm: "",
    booleanTagNames: [],
};

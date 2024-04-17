import { type Concert } from "src/lib/bindings/Concert";

export type FiltersType = {
    searchTerm: string;
    wigmoreU35: boolean;
};

// Check if a concert satisfies the filters
export function satisfies(concert: Concert, filters: FiltersType): boolean {
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

    // Check U35 filter
    let u35Pass = filters.wigmoreU35 ? concert.is_wigmore_u35 : true;

    return searchPass && u35Pass;
}


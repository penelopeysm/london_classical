<script lang="ts">
    import { type Concert } from "src/types";

    export let selectedConcert: Concert | null;
</script>

<div class="details">
    {#if selectedConcert !== null}
        <h3>{selectedConcert.title}</h3>
        <a href={selectedConcert.url}>Link to venue's website</a>
        <p>{selectedConcert.datetime.toLocaleString()}</p>
        <p>{selectedConcert.venue}</p>
        <p>{selectedConcert.performers.join(", ")}</p>

        <p>
            “I tell you I must go!” I retorted, roused to something like
            passion. “Do you think I can stay to become nothing to you? Do you
            think I am an automaton?—a machine without feelings? and can bear to
            have my morsel of bread snatched from my lips, and my drop of living
            water dashed from my cup? Do you think, because I am poor, obscure,
            plain, and little, I am soulless and heartless? You think wrong!—I
            have as much soul as you,—and full as much heart! And if God had
            gifted me with some beauty and much wealth, I should have made it as
            hard for you to leave me, as it is now for me to leave you. I am not
            talking to you now through the medium of custom, conventionalities,
            nor even of mortal flesh;—it is my spirit that addresses your
            spirit; just as if both had passed through the grave, and we stood
            at God’s feet, equal,—as we are!”
        </p>

        <p>
            “As we are!” repeated Mr. Rochester—“so,” he added, enclosing me in
            his arms, gathering me to his breast, pressing his lips on my lips:
            “so, Jane!”
        </p>

        <p>
            “Yes, so, sir,” I rejoined: “and yet not so; for you are a married
            man—or as good as a married man, and wed to one inferior to you—to
            one with whom you have no sympathy—whom I do not believe you truly
            love; for I have seen and heard you sneer at her. I would scorn such
            a union: therefore I am better than you—let me go!”
        </p>

        <p>“Where, Jane? To Ireland?”</p>

        <p>“Yes—to Ireland. I have spoken my mind, and can go anywhere now.”</p>

        <p>
            “Jane, be still; don’t struggle so, like a wild frantic bird that is
            rending its own plumage in its desperation.”
        </p>

        <p>
            “I am no bird; and no net ensnares me; I am a free human being with
            an independent will, which I now exert to leave you.”
        </p>
    {:else}
        <p>No concert selected</p>
    {/if}
</div>

<style>
    .details {
        width: 50%;
        max-width: 50%;
        height: 100%;
        min-height: 0;
        max-height: 100%;
        overflow-y: auto;
    }

    .details > *:first-child {
        margin-top: 0;
    }

    .details > *:last-child {
        margin-bottom: 0;
    }
</style>

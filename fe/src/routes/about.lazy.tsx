import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/about")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <main className="w-1/3 mx-auto space-y-4">
      <h1 className="text-xl font-bold">About</h1>
      <p>
        There were always times when I needed to share something from my phone
        to my computer or vice versa, so I set up a WhatsApp group with just
        myself in it.
      </p>
      <p>
        I once needed to share a link with other people, I found a website that
        limited you to a maximum of 10 links, and the link expired instantly as
        soon as I added it.
      </p>
      <p>
        That’s why I created this app. It’s not perfect, but at least it makes
        it easy to quickly share a bit of text.
      </p>
      <p>
        It was also an opportunity to discover new concepts and technologies
        (WebSocket and Redis).
      </p>
      <p>
        Just a heads-up: not everything works properly. Messages are fine – if
        you can see your message, it’s been sent successfully – but the rest
        might play up.
      </p>
      <p>
        It’s a project on top of everything else I’m doing on the side; I don’t
        want to spend ages on it and would rather use that time to discover
        other things.
      </p>
      <p>
        The repository remains public, so please feel free to contribute if
        you’d like to improve it.
      </p>
    </main>
  );
}

# Overview

This frontend gave me the chance to practise and deepen my love for TanStack. I learnt how to interact with a WebSocket and gained an introduction to the finer points of rendering a React component (chat infinite scrolling).

It uses React (TypeScript, Vite) with:

- Tanstask Router (file-based) for routing
- Tanstask Query, Axios and Zod for fetching (and parsing)

# Folder structure

```
src/
    components/
        ui/
        modals/
        shared/                           # shared across multiple pages
        scoped/                           # components scoped to a specific page, route or other component
            routes/                       # route-scoped components
                users/
                    self/                 # components only used in /users/self
                        profile_card.tsx
            others/                       # non-route scoped components
                header/
    contexts/
    hooks/                                # TanStack Query hooks only
        users.ts
    routes/                               # TanStack Router file-based routes
        users/
            self.tsx
    shared/
        api.ts                            # centralized API client
        types_and_schemas.ts              # frontend schemas mirrored from backend schemas
        config.ts                         # prod | dev, api url, ...
    main.tsx
```

# conventions

- only use Tanstack Query for hooks
- hooks must be returned by functions defined in `hooks/`
- hooks functions names start with `use` and use camelCase
- every API response must be parsed
- frontend schemas should mirror backend schemas as closely as possible
- only use the exported `api` client for API calls (never use `fetch()` directly)
- for forms, use Tanstack Forms and validate/parse inputs before making API calls
- use Tanstack Router with file-based routing
- loading and error states must always be handled explicitly

# snake case

Yes, I use snake case in TypeScript; I really enjoyed the Rust style of writing and wanted to try applying it to the frontend as well:

- `snake_case`: by default
- `camelCase`: hooks
- `PascalCase`: components, types, classes

# Development setup

It requires [Bun](https://bun.sh/):

```shell
bun install
bun dev
```

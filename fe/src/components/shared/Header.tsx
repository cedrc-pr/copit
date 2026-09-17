import { Link } from "@tanstack/react-router";

export function Header() {
  return (
    <header className="flex justify-between items-center h-20 border-b border-black/50 mb-10">
      <div className="flex gap-8">
        <Link to="/" className="text-xl font-bold">
          Copit
        </Link>
        <p className="mt-0.75 text-neutral-600">Send things fast</p>
      </div>
      <nav className="flex gap-4">
        <Link to={"/about"} className="underline underline-offset-2">
          about
        </Link>
        <a
          href="https://github.com/cedrc-pr"
          target="_blank"
          className="underline underline-offset-2"
        >
          contact
        </a>
        <a
          href="https://github.com/cedrc-pr/copit"
          target="_blank"
          className="underline underline-offset-2"
        >
          github
        </a>
      </nav>
    </header>
  );
}

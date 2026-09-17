import QRCode from "react-qr-code";
import copy_svg from "../../../../img/svg/copy.svg";
import type { Room } from "../../../../shared/types_and_schemas";

type Props = {
  id: Room["id"];
};

export function RoomInfos({ id }: Props) {
  return (
    <div className="space-y-6">
      <h1 className="text-xl font-bold">To join room</h1>
      <QRCode value={window.location.href} className="mx-auto" />
      <div className="flex justify-between items-center">
        <p>{window.location.href}</p>
        <button
          onClick={() => navigator.clipboard.writeText(window.location.href)}
        >
          <img
            src={copy_svg}
            alt="copy"
            title="copy room link"
            className="w-7 p-1"
          />
        </button>
      </div>
      <div className="flex justify-between line-clamp-1 items-center">
        <p>{id}</p>
        <button
          onClick={() => navigator.clipboard.writeText(id)}
          className="hover:cursor-pointer"
        >
          <img
            src={copy_svg}
            alt="copy"
            title="copy room name"
            className="w-7 p-1"
          />
        </button>
      </div>
    </div>
  );
}

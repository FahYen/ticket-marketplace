import React from "react";

type GameCardProps = {
  opponent: string;
  opponentAbbr?: string;
  opponentColor: string; // hex like #532E1F
  days: string;
  hours: string;
  minutes: string;
  price: string;
};

export default function GameCard({
  opponent,
  opponentAbbr,
  opponentColor,
  days,
  hours,
  minutes,
  price,
}: GameCardProps) {
  return (
    <div
      className="w-full rounded-[45px] border border-spartan-dark bg-[#F3F3F3] overflow-hidden cursor-pointer hover:scale-105 transition-transform duration-200"
      style={{ boxShadow: "0 5px 0 0 #191A23" }}
    >
      {/* Desktop: match Figma geometry */}
      <div className="hidden lg:flex h-[235px] items-start px-[50px] pr-[84px] relative">
        {/* LEFT block (top:21, width:429) */}
        <div className="w-[429px] pt-[21px] flex flex-col gap-[40px]">
          {/* Title chips */}
          <div className="flex flex-col items-start">
            <div className="rounded-[7px] bg-spartan-green px-[7px]">
              <span className="text-white font-space-grotesk text-[30px] font-medium leading-normal">
                Michigan State vs.
              </span>
            </div>

            <div
              className="rounded-[7px] px-[7px]"
              style={{ backgroundColor: opponentColor }}
            >
              <span className="text-white font-space-grotesk text-[30px] font-medium leading-normal">
                {opponent}
              </span>
            </div>

            {opponentAbbr ? (
              <div className="px-[7px]">
                <span className="text-black font-space-grotesk text-[30px] font-medium leading-normal">
                  {opponentAbbr}
                </span>
              </div>
            ) : null}
          </div>

          {/* Link row (Ticket starts...) */}
          <div className="flex items-center gap-[15px]">
            <div className="w-[41px] h-[41px] rounded-full bg-spartan-dark flex items-center justify-center flex-shrink-0">
              <svg
                className="block"
                width="20"
                height="18"
                viewBox="0 0 20 18"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="M0.75 4.20098C0.0326046 4.61518 -0.213226 5.53256 0.201046 6.25C0.615318 6.96744 1.53256 7.21327 2.25 6.79902L1.5 5.5L0.75 4.20098ZM20.2694 -4.11176C20.4839 -4.91196 20.009 -5.73449 19.2088 -5.94893L6.16884 -9.44293C5.36864 -9.65737 4.54612 -9.18246 4.33168 -8.38226C4.11724 -7.58206 4.59215 -6.75954 5.39235 -6.5451L16.9834 -3.43928L13.8776 8.15177C13.6632 8.95197 14.1381 9.77449 14.9383 9.98893C15.7385 10.2034 16.561 9.72845 16.7754 8.92825L20.2694 -4.11176ZM1.5 5.5L2.25 6.79902L19.5706 -3.20098L18.8206 -4.5L18.0706 -5.79902L0.75 4.20098L1.5 5.5Z"
                  fill="#B9FF66"
                />
              </svg>
            </div>

            <span className="text-black font-space-grotesk text-[20px] leading-none pt-[1px]">
              Ticket starts at ${price}
            </span>
          </div>
        </div>

        {/* TIMER block: match left:570, top:54, size 397x127 */}
        <div className="absolute left-[570px] top-[54px] w-[397px] h-[127px]">
          <div
            className="flex w-[397px] h-[127px] items-start rounded-[40px] overflow-hidden"
            style={{ backgroundColor: withAlpha(opponentColor, 0.65) }}
          >
            <TimerCell value={days} label="DAYS" withDivider />
            <TimerCell value={hours} label="HOURS" withDivider />
            <TimerCell value={minutes} label="MINS" />
          </div>
        </div>

        {/* FOOTBALL icon: use the ORIGINAL Figma SVG so it doesn't “mess up” */}
        <div className="absolute right-[84px] top-[73px] w-[89px] h-[89px]">
          <FigmaFootballIcon />
        </div>
      </div>

      {/* Mobile/Tablet fallback (stacked, still clean) */}
      <div className="lg:hidden p-6">
        <div className="flex flex-col gap-6">
          <div className="flex flex-col gap-2">
            <div className="inline-block rounded-[7px] bg-spartan-green px-3 py-2 w-fit">
              <span className="text-white font-semibold text-lg leading-none">
                Michigan State vs.
              </span>
            </div>

            <div
              className="inline-block rounded-[7px] px-3 py-2 w-fit"
              style={{ backgroundColor: opponentColor }}
            >
              <span className="text-white font-semibold text-lg leading-none">
                {opponent}
              </span>
            </div>

            {opponentAbbr ? (
              <span className="text-black font-bold text-lg leading-none">
                {opponentAbbr}
              </span>
            ) : null}
          </div>

          <div
            className="flex rounded-[40px] overflow-hidden"
            style={{ backgroundColor: withAlpha(opponentColor, 0.65) }}
          >
            <TimerCell value={days} label="DAYS" withDivider />
            <TimerCell value={hours} label="HOURS" withDivider />
            <TimerCell value={minutes} label="MINS" />
          </div>

          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-full bg-spartan-dark flex items-center justify-center">
              <svg className="block" width="20" height="18" viewBox="0 0 20 18" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path
                  d="M0.75 4.20098C0.0326046 4.61518 -0.213226 5.53256 0.201046 6.25C0.615318 6.96744 1.53256 7.21327 2.25 6.79902L1.5 5.5L0.75 4.20098ZM20.2694 -4.11176C20.4839 -4.91196 20.009 -5.73449 19.2088 -5.94893L6.16884 -9.44293C5.36864 -9.65737 4.54612 -9.18246 4.33168 -8.38226C4.11724 -7.58206 4.59215 -6.75954 5.39235 -6.5451L16.9834 -3.43928L13.8776 8.15177C13.6632 8.95197 14.1381 9.77449 14.9383 9.98893C15.7385 10.2034 16.561 9.72845 16.7754 8.92825L20.2694 -4.11176ZM1.5 5.5L2.25 6.79902L19.5706 -3.20098L18.8206 -4.5L18.0706 -5.79902L0.75 4.20098L1.5 5.5Z"
                  fill="#B9FF66"
                />
              </svg>
            </div>
            <span className="text-black text-base leading-none pt-[1px]">
              Ticket starts at ${price}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}

/** Figma timer cell sizing: 133x123 with padding 10 */
function TimerCell({
  value,
  label,
  withDivider,
}: {
  value: string;
  label: string;
  withDivider?: boolean;
}) {
  return (
    <div className="relative w-[133px] h-[123px] p-[10px] flex flex-col items-center justify-center">
      {withDivider ? (
        <div className="absolute right-0 top-0 h-full w-[2px] opacity-[0.04] bg-[#EBEBEB]" />
      ) : null}

      <div className="text-[#EBEBEB] font-jost text-[50px] font-semibold leading-[62.5px]">
        {value}
      </div>
      <div className="text-white/50 font-jost text-[22px] font-semibold leading-[33px]">
        {label}
      </div>
    </div>
  );
}

/** Use the original Figma football SVG (89x89). This avoids the “messed up” icon. */
function FigmaFootballIcon() {
  return (
    <svg
      width="89"
      height="89"
      viewBox="0 0 89 89"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className="block"
      preserveAspectRatio="xMidYMid meet"
    >
      <path
        d="M86.7273 2.69408C86.586 2.12915 86.1623 1.70545 85.5974 1.56422C85.1737 1.42299 73.8751 -0.554263 59.7519 0.151898C40.8268 1.14052 25.8562 6.64858 16.2524 16.2524C6.64859 25.8562 1.14052 40.8268 0.151898 59.7519C-0.554263 73.8751 1.42299 85.1737 1.56422 85.5974C1.70545 86.1623 2.12915 86.586 2.69408 86.7272C3.11778 86.8685 11.168 88.2808 22.3254 88.2808C24.3026 88.2808 26.4211 88.2808 28.5396 88.1396C28.822 88.1396 29.1045 88.1396 29.387 88.1396C29.5282 88.2808 29.8107 88.2808 30.0931 88.2808C30.3756 88.2808 30.6581 88.1396 30.9405 87.9983C48.7358 86.7273 63.0002 81.2192 72.1803 72.0391C81.7841 62.4353 87.2922 47.4647 88.2808 28.5396C88.8457 14.4164 86.7273 3.11777 86.7273 2.69408Z"
        fill="#231F20"
      />
      <path
        d="M52.1253 34.3301L49.0182 37.4372L46.6173 35.0362C46.0524 34.4713 45.2049 34.4713 44.64 35.0362C44.0751 35.6012 44.0751 36.4486 44.64 37.0135L47.041 39.4144L44.0751 42.3803L41.6742 39.9794C41.1092 39.4144 40.2618 39.4144 39.6969 39.9794C39.132 40.5443 39.132 41.3917 39.6969 41.9566L42.0979 44.3576L39.132 47.3234L36.731 44.9225C36.1661 44.3576 35.3187 44.3576 34.7538 44.9225C34.1888 45.4874 34.1888 46.3348 34.7538 46.8997L37.1547 49.3007L34.0476 52.4078C33.4827 52.9727 33.4827 53.8201 34.0476 54.385C34.3301 54.6675 34.6125 54.8087 35.0362 54.8087C35.4599 54.8087 35.7424 54.6675 36.0249 54.385L39.132 51.2779L41.5329 53.6789C41.8154 53.9613 42.0978 54.1026 42.5215 54.1026C42.804 54.1026 43.2277 53.9613 43.5102 53.6789C44.0751 53.114 44.0751 52.2666 43.5102 51.7016L41.1092 49.3007L44.0751 46.3348L46.4761 48.7358C46.7585 49.0182 47.041 49.1595 47.4647 49.1595C47.7471 49.1595 48.1708 49.0182 48.4533 48.7358C49.0182 48.1708 49.0182 47.3234 48.4533 46.7585L46.0523 44.3576L49.0182 41.3917L51.4192 43.7926C51.7017 44.0751 51.9841 44.2163 52.4078 44.2163C52.8315 44.2163 53.114 44.0751 53.3964 43.7926C53.9614 43.2277 53.9614 42.3803 53.3964 41.8154L50.9955 39.4144L54.1026 36.3073C54.6675 35.7424 54.6675 34.895 54.1026 34.3301C53.5377 33.7651 52.6903 33.7651 52.1253 34.3301Z"
        fill="#231F20"
      />
    </svg>
  );
}

/** Convert "#RRGGBB" to rgba with alpha, e.g. alpha=0.65 */
function withAlpha(hex: string, alpha: number) {
  const normalized = hex.replace("#", "").trim();
  const r = parseInt(normalized.slice(0, 2), 16);
  const g = parseInt(normalized.slice(2, 4), 16);
  const b = parseInt(normalized.slice(4, 6), 16);
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}
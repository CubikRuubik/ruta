import { FC, useEffect, useRef, useState } from "react";
import { useTheme } from "../hooks/useTheme";

export const Settings: FC = () => {
  const [settingsOpen, setSettingsOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  const { toggle } = useTheme();

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setSettingsOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  return (
    <div className="absolute top-4 right-4">
      <button
        title="Settings"
        className="cursor-pointer"
        onClick={(e) => {
          e.stopPropagation();
          setSettingsOpen((prev) => !prev);
        }}
      >
        ⚙️
      </button>

      {settingsOpen && (
        <div className="absolute right-0 mt-2 w-40 border rounded shadow-lg z-10 min-h-10 bg-(--card) text-(--card-foreground) dark:bg-(--card) dark:text-(--card-foreground)">
          <button
            className="w-full px-4 py-2"
            onClick={() => {
              toggle();
              setSettingsOpen(false);
            }}
          >
            Toggle Theme
          </button>
        </div>
      )}
    </div>
  );
};

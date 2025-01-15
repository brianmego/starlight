import React from "react";
export const ListboxWrapper = ({children}: {children: React.ReactNode}) => (
  <div className="min-w-[260px] w-fit border-small px-1 py-2 rounded-small border-default-200 dark:border-default-100">
    {children}
  </div>
);

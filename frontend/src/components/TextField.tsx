import React from "react";

type TextFieldProps = {
  label: string;
  value: string;
  onChange: (value: string) => void;
};

export function TextField({ label, value, onChange }: TextFieldProps) {
  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
      <label>{label}</label>
      <input
        type="text"
        value={value}
        onChange={(e) => onChange(e.target.value)}
      />
    </div>
  );
}

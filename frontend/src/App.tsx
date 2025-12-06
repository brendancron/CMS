import React, { useState } from 'react';
import './App.css';
import { TextField } from './components/TextField';

function App() {
  const [form, setForm] = useState({
    title: "",
    subtitle: "",
    author: "",
  });
  const update = (key: keyof typeof form) => (value: string) =>
    setForm((f) => ({ ...f, [key]: value }));

  const handlePrint = () => {
    console.log("Form values:", form);
  };
  return (
    <div style={{ padding: 20, display: "flex", flexDirection: "column", gap: 12 }}>
      <TextField label="Title" value={form.title} onChange={update("title")} />
      <TextField label="Subtitle" value={form.subtitle} onChange={update("subtitle")} />
      <TextField label="Author" value={form.author} onChange={update("author")} />

      <button onClick={handlePrint}>Print Values</button>
    </div>
  );
}

export default App;

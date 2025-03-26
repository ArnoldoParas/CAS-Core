import { useState, useEffect } from "react";
import "./App.css";
import Sidebar from "./components/Sidebar";
import General from "./components/General";
import PDF from "./components/PDF";
import Etiquetas from "./components/Etiquetas";

function App() {
  const [currentView, setCurrentView] = useState("general");
  const [theme, setTheme] = useState(() => localStorage.getItem("theme") || "theme-light");

  useEffect(() => {
    localStorage.setItem("theme", theme);
  }, [theme]);

  const changeView = (view) => {
    setCurrentView(view);
  };

  const toggleTheme = () => {
    setTheme(prevTheme => (prevTheme === "theme-light" ? "theme-dark" : "theme-light"));
  };

  return (
    <main className={`container ${theme}`}>
      <Sidebar currentView={currentView} changeView={changeView} toggleTheme={toggleTheme}  />
      <div className="work_space">
        {currentView === "general" && <General />}
        {currentView === "pdf" && <PDF />}
        {currentView === "etiquetas" && <Etiquetas />}
      </div>
    </main>
  );
}

export default App;

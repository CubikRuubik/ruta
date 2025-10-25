import "./App.css";
import Dashboard from "./components/Dashboard";
import { Settings } from "./components/Settings";

function App() {
  return (
    <main className="w-full h-full ">
      <div className="bg-background text-foreground min-h-screen p-4">
        <Settings />
        <Dashboard />
      </div>
    </main>
  );
}

export default App;

import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="h-screen flex flex-col bg-gray-50 dark:bg-gray-900">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-6 py-4">
        <h1 className="text-2xl font-semibold text-gray-900 dark:text-gray-100">
          ReqSmith
        </h1>
        <p className="text-sm text-gray-600 dark:text-gray-400">
          Modern Requirements Management
        </p>
      </header>

      {/* Main Content */}
      <main className="flex-1 flex items-center justify-center p-8">
        <div className="max-w-md w-full space-y-8">
          <div className="text-center">
            <h2 className="text-3xl font-bold text-gray-900 dark:text-gray-100">
              Welcome to ReqSmith
            </h2>
            <p className="mt-2 text-gray-600 dark:text-gray-400">
              Phase 0: Project Setup Complete ✓
            </p>
          </div>

          {/* Demo Greeting (will be replaced) */}
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6 space-y-4">
            <form
              onSubmit={(e) => {
                e.preventDefault();
                greet();
              }}
              className="space-y-3"
            >
              <input
                type="text"
                value={name}
                onChange={(e) => setName(e.currentTarget.value)}
                placeholder="Enter your name..."
                className="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg 
                         bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
                         focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none"
              />
              <button
                type="submit"
                className="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium 
                         rounded-lg transition-colors duration-150"
              >
                Test Connection
              </button>
            </form>
            {greetMsg && (
              <p className="text-center text-green-600 dark:text-green-400 font-medium">
                {greetMsg}
              </p>
            )}
          </div>

          {/* Next Steps */}
          <div className="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
            <h3 className="font-semibold text-blue-900 dark:text-blue-100 mb-2">
              Next Steps
            </h3>
            <ul className="text-sm text-blue-800 dark:text-blue-200 space-y-1">
              <li>• Phase 1: Implement ReqIF data model</li>
              <li>• Phase 2: Build parser and serializer</li>
              <li>• Phase 3: Create three-pane UI layout</li>
            </ul>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 px-6 py-3">
        <p className="text-xs text-gray-500 dark:text-gray-400 text-center">
          ReqSmith v0.1.0 - Built with Rust + Tauri + React
        </p>
      </footer>
    </div>
  );
}

export default App;

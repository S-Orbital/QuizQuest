import React from 'react'
import './App.css'
import logo from './logo.svg'

function MyHeader() {
  return (
    <header>
      <h1>QuizQuest</h1>
    </header>
  );
}

function MyContent() {
  return(
    <main>
      <p>
        Welcome to QuizQuest! There is nothing here yet...
      </p>
      <ul>
        <li>Testing lists #1</li>
        <li>Testing lists #2</li>
      </ul>
      <img src={logo} alt="My logo for testing purposes" height="500em" width="auto" />
    </main>
  )
}

function App() {
  return (
    <div className="App">
      <MyHeader />
      <MyContent />
    </div>
  );
}

export default App;
import {FC, useState, useEffect} from 'react';
import {init, solve} from 'blockudokill-solver';

// import './App.module.css';

const App: FC<{}> = () => {
  const [solverInitialized, setSolverInitialized] = useState<boolean>(false);

  useEffect(() => {
    init().then(() => {
      setSolverInitialized(true)
    })
  }, []);

  useEffect(() => {
    if (solverInitialized) {
      const gameboard = new Uint8Array(0).fill(0);
      const block1 = new Uint8Array(1).fill(0);
      const block2 = new Uint8Array(1).fill(0);
      const block3 = new Uint8Array(1).fill(0);
      console.log(solve(gameboard, block1, block2, block3));
    }
  }, [solverInitialized]);

  return <div className="App">Hello</div>;
};

export default App;

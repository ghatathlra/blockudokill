import {FC, useState, useEffect, useCallback} from 'react';
import {init, solve} from 'blockudokill-solver';

import {Placement, SolveResult} from 'types';
import {Block} from 'objects';

import BlocksForm from 'components/BlocksForm/BlocksForm';
import BlocksPreview from 'components/BlocksPreview/BlocksPreview';
import GameBoard from 'components/GameBoard/GameBoard';

import styles from './App.module.css';

const App: FC<{}> = () => {
  const [solverInitialized, setSolverInitialized] = useState<boolean>(false);

  const [selectedBlocks, setSelectedBlocks] = useState<Block[]>([]);
  const [placements, setPlacements] = useState<Placement[]>([]);
  const [currentGameState, setCurrentGameState] = useState<number[]>([]);
  const [nextGameState, setNextGameState] = useState<number[]>([]);

  useEffect(() => {
    init().then(() => {
      setSolverInitialized(true);
    });
  }, []);

  const handleSolve = useCallback(() => {
    if (solverInitialized) {
      if (selectedBlocks.length === 3) {
        setCurrentGameState(nextGameState);
        const gameboard = Uint8Array.from(nextGameState);
        const block1 = Uint8Array.from(selectedBlocks[0].getCells());
        const block2 = Uint8Array.from(selectedBlocks[1].getCells());
        const block3 = Uint8Array.from(selectedBlocks[2].getCells());
        const solveResult: SolveResult = solve(gameboard, block1, block2, block3);
        setPlacements(solveResult.placements);
        setNextGameState(solveResult.next_state);
        setSelectedBlocks([]);
        console.log(solveResult);
      } else {
        alert('Please select 3 blocks!');
      }
    } else {
      alert('Solver is not initialized!');
    }
  }, [solverInitialized, selectedBlocks, nextGameState]);

  return (
    <div className={styles.app}>
      <div className={styles.playground}>
        <div className={styles.gameboardouter}>
          <div className={styles.gameboardinner}>
            <GameBoard gameState={currentGameState} placements={placements} />
            <button type="button" className={styles.submitbutton} onClick={handleSolve}>
              Next &gt;&gt;
            </button>
          </div>
        </div>
        <div className={styles.blockspreview}>
          <BlocksPreview value={selectedBlocks} onChange={setSelectedBlocks} />
        </div>
      </div>
      <div className={styles.blocksform}>
        <BlocksForm value={selectedBlocks} onChange={setSelectedBlocks} />
      </div>
    </div>
  );
};

export default App;

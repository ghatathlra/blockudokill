import {FC, useState, useEffect, useCallback} from 'react';
// import {init, solve, calc_score} from 'blockudokill-solver';
import {init, solve} from 'blockudokill-solver';

import {Placement, SolveResult} from 'types';
import {Block} from 'objects';

import BlocksForm from 'components/BlocksForm/BlocksForm';
import BlocksPreview from 'components/BlocksPreview/BlocksPreview';
import GameBoard from 'components/GameBoard/GameBoard';

import styles from './App.module.css';

const App: FC<{}> = () => {
  const [solverInitialized, setSolverInitialized] = useState<boolean>(false);
  const [placementsViewMode, setPlacementsViewMode] = useState<boolean>(true);
  const [selectedBlocks, setSelectedBlocks] = useState<Block[]>([]);
  const [placements, setPlacements] = useState<Placement[]>([]);
  const [previousGameState, setPreviousGameState] = useState<number[]>([]);
  const [currentGameState, setCurrentGameState] = useState<number[]>([]);

  useEffect(() => {
    init().then(() => {
      setSolverInitialized(true);
    });
  }, []);

  // useEffect(() => {
  //   if (solverInitialized) {
  //     console.log(calc_score(Uint8Array.from(currentGameState)));
  //   }
  // }, [solverInitialized, currentGameState]);

  const handleGameBoardCellClick = useCallback((i: number) => {
    if (!placementsViewMode) {
      if (currentGameState.includes(i)) {
        setCurrentGameState(currentGameState.filter((cell) => cell !== i));
      } else {
        setCurrentGameState([...currentGameState, i]);
      }
    }
  }, [placementsViewMode, currentGameState]);

  const handleSwitchViewMode = useCallback(() => {
    setPlacementsViewMode((placementsViewMode) => !placementsViewMode);
  }, []);

  const handleSolve = useCallback(() => {
    if (solverInitialized) {
      if (selectedBlocks.length === 3) {
        setPreviousGameState(currentGameState);
        const gameboard = Uint8Array.from(currentGameState);
        const block1 = Uint8Array.from(selectedBlocks[0].getCells());
        const block2 = Uint8Array.from(selectedBlocks[1].getCells());
        const block3 = Uint8Array.from(selectedBlocks[2].getCells());
        const solveResult: SolveResult = solve(gameboard, block1, block2, block3);
        setPlacements(solveResult.placements);
        setCurrentGameState(solveResult.next_state);
        setSelectedBlocks([]);
        console.log(solveResult);
      } else {
        alert('Please select 3 blocks!');
      }
    } else {
      alert('Solver is not initialized!');
    }
  }, [solverInitialized, selectedBlocks, currentGameState]);

  return (
    <div className={styles.app}>
      <div className={styles.playground}>
        <div className={styles.gameboardouter}>
          <div className={styles.gameboardinner}>
            <GameBoard
              gameState={placementsViewMode ? previousGameState : currentGameState}
              placements={placementsViewMode ? placements : []}
              onCellClick={handleGameBoardCellClick}
            />
            <div className={styles.gameboardcontrol}>
              <button type="button" className={styles.viewmodebutton} onClick={handleSwitchViewMode}>
                {placementsViewMode ? 'Placements View' : 'State View'}
              </button>
              <button type="button" className={styles.submitbutton} onClick={handleSolve}>
                Next &gt;&gt;
              </button>
            </div>
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

import {FC} from 'react';
import {Placement} from 'types';
import {GameBoardCellColors} from 'enums';
import {AllStripedCells} from 'objects';

import styles from './GameBoard.module.css';

type Props = {
  gameState: number[];
  placements: Placement[];
  onCellClick: (i: number) => void;
};

function getCellColor(i: number, gameState: number[], placements: Placement[]): string {
  let finalColor = GameBoardCellColors.empty;
  const placementColors = [GameBoardCellColors.blockOne, GameBoardCellColors.blockTwo, GameBoardCellColors.blockThree];
  if (AllStripedCells.STRIPED_CELLS.includes(i)) {
    finalColor = GameBoardCellColors.striped;
  }
  if (gameState.includes(i)) {
    finalColor = GameBoardCellColors.occupied;
  }
  for (let blcki = placements.length - 1; blcki >= 0; blcki -= 1) {
    const placement = placements[blcki];
    if (placement.block.map((bi) => bi + placement.position).includes(i)) {
      finalColor = placementColors[blcki] || GameBoardCellColors.occupied;
    }
  }
  return finalColor;
}

const GameBoard: FC<Props> = (props) => {
  const {gameState, placements, onCellClick} = props;
  return (
    <div className={styles.gameboard}>
      {Array.from(new Array(81).keys()).map((i) => (
        <div
          key={i}
          className={styles.cell}
          style={{backgroundColor: getCellColor(i, gameState, placements)}}
          onClick={() => {
            onCellClick(i);
          }}
        />
      ))}
    </div>
  );
};

export default GameBoard;

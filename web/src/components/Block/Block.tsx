import {FC, useMemo} from 'react';
import {Block} from 'objects';

import styles from './Block.module.css';

type Props = {
  block: Block;
};

function arrFromZeroToN(n: number): number[] {
  if (n > 0) {
    return Array.from(new Array(n).keys());
  } else {
    return [];
  }
}

const BlockComponent: FC<Props> = (props) => {
  const {block} = props;
  const cells = block.getCells();
  const highlightCells = useMemo(
    () =>
      cells.reduce((c: Record<string, boolean>, n) => {
        let x = n % 9;
        let y = Math.floor(n / 9);
        c[String(y * block.getWidth() + x)] = true;
        return c;
      }, {}),
    [cells, block],
  );
  return (
    <div className={styles.block} style={{gridTemplateColumns: `repeat(${block.getWidth()}, 1fr)`}}>
      {arrFromZeroToN(block.getWidth() * block.getHeight()).map((i) => (
        <div key={i} className={styles.cell + (highlightCells[i] ? `  ${styles.highlight}` : '')} />
      ))}
    </div>
  );
};

export default BlockComponent;

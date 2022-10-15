import {FC, useCallback} from 'react';
import {AllBlocks, Block} from 'objects';

import BlockComponent from 'components/Block/Block';
import styles from './BlocksForm.module.css';

type Props = {
  value: Block[];
  onChange: (val: Block[]) => void;
};

const BlocksForm: FC<Props> = (props) => {
  const {value, onChange} = props;

  const handleSelectBlock = useCallback(
    (block: Block) => {
      if (value.length < 3) {
        const newValue = [...value, block];
        onChange(newValue);
      }
    },
    [value, onChange],
  );

  return (
    <div className={styles.blocksform}>
      {AllBlocks.BLOCKS.map((block, i) => (
        <div
          key={i}
          className={styles.blockcontainer}
          onClick={() => {
            handleSelectBlock(block);
          }}
        >
          <BlockComponent block={block} />
        </div>
      ))}
    </div>
  );
};

export default BlocksForm;

import {FC, useCallback} from 'react';
import {Block} from 'objects';

import BlockComponent from 'components/Block/Block';
import styles from './BlocksPreview.module.css';

type Props = {
  value: Block[];
  onChange: (val: Block[]) => void;
};

const BlocksPreview: FC<Props> = (props) => {
  const {value, onChange} = props;

  const handleDeleteBlock = useCallback(
    (block: Block) => {
      const newValue = value.filter((blck) => blck.getSignature() !== block.getSignature());
      onChange(newValue);
    },
    [value, onChange],
  );

  return (
    <div className={styles.blockspreview}>
      {value.map((block, i) => (
        <div
          key={i}
          className={styles.blockcontainer}
          onClick={() => {
            handleDeleteBlock(block);
          }}
        >
          <BlockComponent block={block} />
        </div>
      ))}
    </div>
  );
};

export default BlocksPreview;

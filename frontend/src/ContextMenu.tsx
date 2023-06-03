import { useState, useEffect } from 'react';
import { Popup, Menu } from 'semantic-ui-react';

import { presentation } from '~/go/models';
import { ListCandidates } from '~/go/presentation/App';

export type ContextMenuProps = {
  open: boolean;
  x: number;
  y: number;

  onMenuClose: () => void;
  onMenuClick: (type: string, pos: presentation.Position) => void;
};

export const ContextMenu: React.FC<ContextMenuProps> = ({
  open,
  x,
  y,
  onMenuClose,
  onMenuClick,
}) => {
  const [cands, setCands] = useState<presentation.Candidate[]>([]);

  useEffect(() => {
    (async () => {
      const cands = await ListCandidates();
      setCands(cands);
    })();
  }, []);

  return (
    <Popup
      basic
      // TODO: this implementation is a bit hacky, so try following two tasks:
      // 1. fix by using `useRef` correctly,
      // 2. or, fix it on upstream if the useRef approach is not possible.
      context={
        {
          getBoundingClientRect: () => ({
            left: x,
            top: y,
            right: x + 1,
            bottom: y + 1,

            height: 0,
            width: 0,
          }),
        } as HTMLElement
      }
      open={open}
      onClose={onMenuClose}
    >
      <Menu secondary vertical visible={open}>
        {cands.map(({ kind, label }) => (
          <Menu.Item
            key={kind}
            onClick={() => {
              const pos: presentation.Position = { x, y };
              onMenuClick(kind, pos);
              onMenuClose();
            }}
          >
            {label}
          </Menu.Item>
        ))}
      </Menu>
    </Popup>
  );
};

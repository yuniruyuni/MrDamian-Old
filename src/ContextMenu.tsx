import { useState, useEffect } from 'react';
import { Dropdown, Portal, Segment } from 'semantic-ui-react'
import { Position, Candidate, candidates } from './bindings';

export type ContextMenuProps = {
  open: boolean;
  x: number;
  y: number;

  onMenuClose: () => void;
  onMenuClick: (type: string, pos: Position) => void;
};

export const ContextMenu: React.FC<ContextMenuProps> = ({open, x, y, onMenuClose, onMenuClick}) => {
  const [ cands, setCands ] = useState<Candidate[]>([]);

  useEffect(() => {
   (async () => {
      const cands = await candidates();
      setCands(cands);
   })()
  }, []);

  // TODO: avoid ad-hock mouse position fixing.
  const AdhockFixForMousePosDiffX = -8;
  const AdhockFixForMousePosDiffY = -16;

  return (
    <Portal open={open} onClose={onMenuClose}>
      <Segment style={{
        position: 'absolute',
        left: x + AdhockFixForMousePosDiffX,
        top: y + AdhockFixForMousePosDiffX,
      }}>
        <Dropdown.Menu visible={open}>
          {cands.map(({kind, label}) => (
            <Dropdown.Item onClick={() => {
              const pos: Position = {
                x: x + AdhockFixForMousePosDiffX,
                y: y + AdhockFixForMousePosDiffY,
              };
              onMenuClick(kind, pos);
              onMenuClose();
            }}>{label}</Dropdown.Item>
          ))}
        </Dropdown.Menu>
      </Segment>
    </Portal>
  );
}

import { useState, useEffect } from 'react';
import { Dropdown, Portal, Segment } from 'semantic-ui-react'
import { Node, Component, components } from './bindings';

export type ContextMenuProps = {
  open: boolean;
  x: number;
  y: number;

  onMenuClose: () => void;
  onMenuClick: (node: Node) => void;
};

export const ContextMenu: React.FC<ContextMenuProps> = ({open, x, y, onMenuClose, onMenuClick}) => {
  const [ comps, setComps ] = useState<Component[]>([]);

  useEffect(() => {
   (async () => {
      const comps = await components();
      setComps(comps);
   })()
  }, [setComps]);

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
          {comps.map(({type, label, inputs, outputs}) => (
            <Dropdown.Item onClick={() => {
              const node: Node = {
                type,
                id: `${Math.random()}`,
                position: { x: x + AdhockFixForMousePosDiffX, y: y + AdhockFixForMousePosDiffY},
                data: {
                  label,
                  inputs: inputs,
                  outputs: outputs,
                },
              };
              onMenuClick(node);
              onMenuClose();
            }}>{label}</Dropdown.Item>
          ))}
        </Dropdown.Menu>
      </Segment>
    </Portal>
  );
}

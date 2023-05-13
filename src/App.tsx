import { useState, useEffect, useCallback } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
} from 'reactflow';

import 'reactflow/dist/style.css';

import { Button, Dropdown, Portal, Segment } from 'semantic-ui-react'

import { usePipeline } from "./pipeline";
import { Node, Component, components } from './bindings';

type Menu = {
  open: boolean;
  x: number;
  y: number;
};

function App() {
  const { onApply, addNode, ...pipeline } = usePipeline();

  const [ menu, setMenu ] = useState<Menu>({open: false, x: 0, y: 0});
  const [ comps, setComps ] = useState<Component[]>([]);

  useEffect(() => {
   (async () => {
      const comps = await components();
      setComps(comps);
   })()
  }, [setComps]);

  const onMenuClose = useCallback(() => { setMenu({open: false, x: 0, y: 0}) }, [setMenu]);

  return (
    <div className="container">
      <ReactFlow
        {...pipeline}
        onPaneContextMenu={ (e) => { setMenu({open: true, x: e.clientX, y: e.clientY}); } }
      >
        <MiniMap />
        <Controls />
        <Background />
      </ReactFlow>
      <Button onClick={onApply} primary>Apply</Button>
      <Portal open={menu.open} onClose={onMenuClose}>
        {/* TODO: avoid ad-hock mouse position fixing. */}
        <Segment style={{position: 'absolute', left: menu.x - 8, top: menu.y - 16}}>
          <Dropdown.Menu visible={menu.open}>
            {comps.map(({type, label, inputs, outputs}) => (
              <Dropdown.Item onClick={() => {
                const node: Node = {
                  type,
                  id: `${Math.random()}`,
                  position: { x: menu.x - 8, y: menu.y - 16 },
                  data: {
                    label,
                    inputs: inputs,
                    outputs: outputs,
                  },
                };
                addNode(node);
                setMenu({open: false, x: 0, y: 0});
              }}>{label}</Dropdown.Item>
            ))}
          </Dropdown.Menu>
        </Segment>
      </Portal>
    </div>
  );
}

export default App;

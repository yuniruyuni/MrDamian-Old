import { useState, useMemo, useCallback } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
} from 'reactflow';

import 'reactflow/dist/style.css';

import { Button, Dropdown, Portal, Segment } from 'semantic-ui-react'

import { usePipeline } from "./pipeline";
import { Node } from './bindings';

type Menu = {
  open: boolean;
  x: number;
  y: number;
};

function App() {
  const { onApply, addNode, ...pipeline } = usePipeline();

  const [ menu, setMenu ] = useState<Menu>({open: false, x: 0, y: 0});

  type Component = {name: string, construct: () => Omit<Node, 'id' | 'position'>};

  const components: Component[] = useMemo(() => [
    {name: 'Twitch Subscriber', construct: () =>  ({
      type: 'TwitchSubscriber',
      data: {
        label: 'Twitch Subscriber',
        inputs: [],
        outputs: [
          {name: 'raid'},
        ],
      }
    })},
    {name: 'Twitch Publisher', construct: () => ({
      type: 'TwitchPublisher',
      data: {
        label: 'Twitch Publisher',
        inputs: [
          {name: 'message'},
        ],
        outputs: [],
      }
    })},
  ], []);

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
            {components.map(({name, construct}) => (
              <Dropdown.Item onClick={() => {
                const node: Node = {
                  ...construct(),
                  id: `${Math.random()}`,
                  position: { x: menu.x - 8, y: menu.y - 16 },
                };
                addNode(node);
                setMenu({open: false, x: 0, y: 0});
              }}>{name}</Dropdown.Item>
            ))}
          </Dropdown.Menu>
        </Segment>
      </Portal>
    </div>
  );
}

export default App;

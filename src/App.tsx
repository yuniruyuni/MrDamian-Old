import { useState, useMemo } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
} from 'reactflow';

import 'reactflow/dist/style.css';

import { Button, Dropdown, Portal, Segment } from 'semantic-ui-react'

import { usePipeline } from "./pipeline";

type Menu = {
  open: boolean;
  x: number;
  y: number;
};

function App() {
  const { onApply, ...pipeline } = usePipeline();

  const [ menu, setMenu ] = useState<Menu>({open: false, x: 0, y: 0});

  type Component = {name: string};

  const components: Component[] = useMemo(() => [
    {name: 'TwitchSubscriber'},
    {name: 'TwitchPublisher'},
  ], []);

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
      <Portal open={menu.open} onClose={() => { setMenu({open: false, x: 0, y: 0}) }}>
        {/* TODO: avoid ad-hock mouse position fixing. */}
        <Segment style={{position: 'absolute', left: menu.x - 8, top: menu.y - 16}}>
          <Dropdown.Menu visible={menu.open}>
            {components.map(({name}) => (<Dropdown.Item>{name}</Dropdown.Item>))}
          </Dropdown.Menu>
        </Segment>
      </Portal>
    </div>
  );
}

export default App;

import { useState, useCallback } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
} from 'reactflow';

import 'reactflow/dist/style.css';

import { Button, Segment, Sidebar, Form, Label } from 'semantic-ui-react'

import { usePipeline } from "./pipeline";

import { Node, InputPort, OutputPort, Position, createComponent } from './bindings';
import { ContextMenu } from "./ContextMenu";

export type ContextMenuState = {
  open: boolean;
  x: number;
  y: number;
};

type SidebarState = {
  open: boolean;
  input?: InputPort;
  output?: OutputPort;
  node?: Node;
};

function App() {
  const [ sidebar, setSidebar ] = useState<SidebarState>({open: false});
  const { onApply, ...pipeline } = usePipeline(
    (node: Node, input: InputPort) => { setSidebar({open: true, input, node }); },
    (node: Node, output: OutputPort) => { setSidebar({open: true, output, node }); },
  );

  const [ menu, setMenu ] = useState<ContextMenuState>({open: false, x: 0, y: 0});
  const onPaneContextMenu = useCallback((e: React.MouseEvent) => { setMenu({open: true, x: e.clientX, y: e.clientY}); }, [setMenu]);
  const onMenuClose = useCallback(() => { setMenu({open: false, x: 0, y: 0}) }, [setMenu]);
  const onMenuClick = useCallback(async (type: string, pos: Position) => {
    await createComponent(type, pos);
  }, [setMenu]);

  return (
    <div className="container">
      <ReactFlow
          {...pipeline}
          fitView={true}
          onPaneContextMenu={onPaneContextMenu}
        >
          <MiniMap />
          <Controls />
          <Background />
      </ReactFlow>
      <Button onClick={onApply} primary>Apply</Button>
      <ContextMenu
        onMenuClose={onMenuClose}
        onMenuClick={onMenuClick}
        {...menu}
      />
      <Sidebar
        as={Segment}
        animation='overlay'
        icon='labeled'
        direction='left'
        inverted
        vertical
        visible={sidebar.open}
        onHide={() => setSidebar({open: false})}
      >
        <div style={{textAlign: 'left', padding: '16px'}}>
          <span>TODO: implement</span>
        </div>
      </Sidebar>
    </div>
  );
}

export default App;

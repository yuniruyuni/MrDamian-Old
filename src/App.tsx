import { useState, useCallback } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
  Edge as RFEdge,
} from 'reactflow';

import 'reactflow/dist/style.css';

import { Button, Container, Modal, Table, Select } from 'semantic-ui-react'

import { usePipeline } from "./pipeline";

import { Node, Edge, InputPort, OutputPort, Position, createComponent } from './bindings';
import { ContextMenu } from "./ContextMenu";
import { AssignmentModal } from "./AssignmentModal";

export type ContextMenuState = {
  open: boolean;
  x: number;
  y: number;
};

type AssignModalState = {
  open: boolean;
  input?: InputPort;
  output?: OutputPort;
  edge?: Edge;
};

function App() {
  const [ modal, setModal ] = useState<AssignModalState>({open: false});
  const { onApply, ...pipeline } = usePipeline(
    (source: Node, target: Node, sourcePort: OutputPort, targetPort: InputPort) => {
      setModal({open: true, input: sourcePort, output: targetPort});
    },
  );

  const onAssign = useCallback((assignment: Record<string, string>) => {
    // TODO: call remote method to save assignment to editor state.
    setModal({open: false});
  }, []);

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
      <AssignmentModal
        {...modal}
        onAssign={onAssign}
        onDiscard={() => setModal({open: false})}
      />
    </div>
  );
}

export default App;

import { useCallback, useState } from 'react';
import 'reactflow/dist/style.css';

import { Button, Modal, Table, Select } from 'semantic-ui-react'

import { Edge, InputPort, OutputPort } from './bindings';

type Prop = {
  open: boolean;
  input?: InputPort;
  output?: OutputPort;
  edge?: Edge;

  onAssign: (assignment: Record<string, string>) => void;
  onDiscard: () => void;
};

export const AssignmentModal: React.FC<Prop> = ({open, input, output, edge, onAssign, onDiscard}) => {
  const [assignment, setAssignment] = useState<Record<string, string>>(edge?.data.assignment ?? {});
  const onClose = onDiscard;
  const onApply = useCallback(() => onAssign(assignment), [onAssign, assignment]);

  return (
    <Modal open={open} onClose={onClose}>
      <Modal.Header>Edit Assignment</Modal.Header>
      <Modal.Content image scrolling>
        <Table>
          <Table.Header>
            <Table.Row>
              <Table.HeaderCell>Target</Table.HeaderCell>
              <Table.HeaderCell>Source</Table.HeaderCell>
            </Table.Row>
          </Table.Header>

          <Table.Body>
            {output?.propertyNames.map((prop: string) => (
              <Table.Row>
                <Table.Cell>
                  {prop}
                </Table.Cell>
                <Table.Cell>
                  <Select
                    value={edge?.data.assignment[prop]}
                    options={
                        input?.
                            propertyNames.
                            map((prop: string) => ({ key: prop, value: prop, text: prop }))
                        ?? []
                    }
                    onChange={(_, {value}) => {
                      setAssignment({
                        ...assignment,
                        [prop]: value as string,
                      })
                    }}
                  />
                </Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table>
      </Modal.Content>
      <Modal.Actions>
        <Button onClick={onClose} secondary>
          Cancel
        </Button>
        <Button onClick={onApply} primary>
          Apply
        </Button>
      </Modal.Actions>
    </Modal>
  );
};

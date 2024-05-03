import React from 'react';
import { Card, CardBody, CardHeader } from '@nextui-org/card';
import { Divider } from '@nextui-org/react';

type Props = {
    post: JSON
}

function BlogPostCard({ post }: Props) {
    let title = (post as any)['title'];
    let summary = (post as any)['summary'];
    let last_update = (post as any)['last_update'];

    return (

        <div>
            {/*todo: to be update*/ }
            {/*<Link href={ `/blog/${ name }` }>*/ }
            <Card>
                <CardHeader>
                    <p className={ 'font-bold' }>{ title }</p>
                </CardHeader>
                <Divider />
                <CardBody>
                    { summary }
                </CardBody>
                <Divider />
                <CardBody>
                    { last_update }
                </CardBody>
            </Card>
            {/*</Link>*/ }
        </div>
    );

}

export default BlogPostCard;

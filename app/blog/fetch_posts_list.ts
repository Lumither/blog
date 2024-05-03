import { Result } from '@/app/public/type/Result';

export default async function fetch_posts_list(): Promise<Result<JSON[], Error>> {
    const response = await fetch(`${ process.env.BLOG_ENDPOINT }/posts/idx.json`);
    if (!response.ok) {
        return {
            ok: false,
            error: new Error(response.statusText)
        };
    }
    const res = await response.json();
    const posts = res['idx'].filter((posts: { type: string }) => posts.type === 'md');
    return {
        ok: true,
        value: posts
    };
}

